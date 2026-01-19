import { describe, expect, it } from 'vitest';

import { classNames, truncateText } from './index';

describe('utils', () => {
  it('truncates text only when needed', () => {
    expect(truncateText('short', 10)).toBe('short');
    expect(truncateText('exact', 5)).toBe('exact');
    expect(truncateText('truncate me', 8)).toBe('truncate...');
  });

  it('joins class names while skipping falsey values', () => {
    expect(classNames('one', undefined, 'two', false, null, 'three')).toBe('one two three');
  });
});
