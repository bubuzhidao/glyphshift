import type { AiTranslationItemInput, AiSkipReason } from './useAiTranslation'
export interface AiFilterPolicy {
  skipPureNumbersOrSymbols: boolean
  skipNumericMeasurements: boolean
  skipSingleCharacter: boolean
  skipTextContainingDigits: boolean
  skipUrls: boolean
  skipEmails: boolean
  skipFilePaths: boolean
  skipShortcuts: boolean
  maxSourceChars: number | null
  excludedPatterns: string[]
}

export const defaultAiFilterPolicy = (): AiFilterPolicy => ({
  skipPureNumbersOrSymbols: true,
  skipNumericMeasurements: true,
  skipSingleCharacter: true,
  skipTextContainingDigits: false,
  skipUrls: true,
  skipEmails: true,
  skipFilePaths: true,
  skipShortcuts: true,
  maxSourceChars: null,
  excludedPatterns: [],
})

const MEASUREMENT_UNITS = new Set([
  'mil', 'mils', 'mm', 'cm', 'um', 'nm', 'µm', 'inch', 'inches', 'px', 'pt',
])

const READOUT_LABELS = new Set([
  'x', 'y', 'z', 'w', 'h', 'grid', 'size', 'xsize', 'ysize', 'area', 'density', 'horizontal',
  'vertical', 'top', 'bottom', 'total', 'components', 'primitives', 'board', 'layer', 'layers',
  'snap', 'distance', 'range', 'pad', 'pads', 'net', 'nets', 'on', 'off', 'and', 'more', 'sq',
  'unit', 'units', 'count', 'width', 'length', 'height', 'radius', 'hole', 'angle', 'type',
  'round', 'outline', 'solder', 'mask', 'paste', 'silkscreen',
])

const GEOMETRY_FIELDS = ['x:', 'y:', 'grid:', 'x-size:', 'y-size:', 'radius:', 'angle1:', 'angle2:', 'hole:']

const COMPONENT_FIELDS = [
  'comment:', 'footprint:', 'manufacturer part:', 'manufacturer:', 'supplier part:', 'mpn:',
  'designator:',
]

const PERCENT_LABELS = ['density', 'opacity', 'scale', 'zoom', 'progress', 'usage']

const COMPONENT_VALUE_SUFFIXES = new Set([
  'ω', 'kω', 'mω', 'ohm', 'ohms', 'kohm', 'uf', 'µf', 'nf', 'pf', 'uh', 'µh', 'nh', 'mh',
])

const NUMERIC_WORDS = new Set(['x', 'fps', 'px', 'pt', 'dpi', 'hz', 'khz', 'mhz', 'ms', 'gb', 'mb', 'kb'])

function containsFieldValue(source: string, key: string): boolean {
  const lower = source.toLowerCase()
  let index = lower.indexOf(key)
  while (index !== -1) {
    const precededByLetter = index > 0 && /\p{L}/u.test(lower[index - 1])
    const followedByValue = lower.slice(index + key.length).trim().length > 0
    if (!precededByLetter && followedByValue) return true
    index = lower.indexOf(key, index + key.length)
  }
  return false
}

function isGeneratedReadout(source: string): boolean {
  if (GEOMETRY_FIELDS.some(key => containsFieldValue(source, key))) return true
  let hasDigit = false
  let unitMeasurements = 0
  let leftoverLetters = 0
  let leftoverWords = 0
  let numberPending = false
  let word = ''
  for (const character of `${source} `) {
    if (/\p{L}/u.test(character)) {
      word += character.toLowerCase()
      continue
    }
    if (word) {
      if (MEASUREMENT_UNITS.has(word)) {
        if (numberPending) unitMeasurements += 1
      } else if (!READOUT_LABELS.has(word)) {
        leftoverLetters += [...word].length
        leftoverWords += 1
      }
      word = ''
      numberPending = false
    }
    if (/\d/.test(character)) {
      hasDigit = true
      numberPending = true
    } else if (!/\s/.test(character)) {
      numberPending = false
    }
  }
  return hasDigit && (unitMeasurements >= 2 || (unitMeasurements === 1 && leftoverLetters <= 4 && leftoverWords <= 2))
}

function isPercentReadout(source: string): boolean {
  return source.includes('%') && PERCENT_LABELS.some(label => containsFieldValue(source, label))
}

function isComponentReadout(source: string): boolean {
  return COMPONENT_FIELDS.some(key => containsFieldValue(source, key))
}

function isComponentValueReadout(source: string): boolean {
  let hasDigit = false
  let suffix = ''
  for (const character of source.trim().toLowerCase()) {
    if (/\d/.test(character)) {
      hasDigit = true
    } else if (!'. ,\''.includes(character)) {
      suffix += character
    }
  }
  return hasDigit && COMPONENT_VALUE_SUFFIXES.has(suffix)
}

function isNumericMeasurement(source: string): boolean {
  if (isComponentReadout(source) || isComponentValueReadout(source)) return true
  if (!/\d/.test(source)) return false
  if (/[/:×*]/.test(source) && /^[\s\d.,+\-/:×*xX]*$/.test(source)) return true
  if (source.endsWith('%') && /^[\s\d.,+\-]*$/.test(source.slice(0, -1))) return true
  const letters = source.replace(/[^\p{L}]/gu, '').toLowerCase()
  if (NUMERIC_WORDS.has(letters)) return true
  return isGeneratedReadout(source) || isPercentReadout(source)
}

export function skipReason(item: AiTranslationItemInput, policy: AiFilterPolicy): AiSkipReason | null {
  const source = item.source.trim()
  if (item.translation?.trim()) return 'already_translated'
  if (item.ignored) return 'ignored'
  if (!source) return 'empty_source'
  if (policy.skipSingleCharacter && [...source].length === 1) return 'single_character'
  if (policy.skipUrls && /^(?:https?|ftp):\/\//i.test(source)) return 'url'
  if (policy.skipEmails && /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(source)) return 'email'
  if (policy.skipFilePaths && /^(?:[a-z]:[\\/]|\\\\|\/)[^\n]+/i.test(source)) return 'file_path'
  if (policy.skipShortcuts && /^(?:(?:ctrl|alt|shift|cmd|command|win|super)\s*\+\s*)+[\w\d]+$/i.test(source)) return 'shortcut'
  if (policy.skipNumericMeasurements && isNumericMeasurement(source)) return 'numeric_measurement'
  if (policy.skipPureNumbersOrSymbols && !/[\p{L}]/u.test(source)) return 'pure_number_or_symbols'
  if (policy.skipTextContainingDigits && /\d/.test(source)) return 'contains_digit'
  if (policy.maxSourceChars && [...source].length > policy.maxSourceChars) return 'too_long'
  if (policy.excludedPatterns.some(pattern => {
    try { return new RegExp(pattern).test(source) }
    catch { return false }
  })) return 'custom_pattern'
  return null
}
