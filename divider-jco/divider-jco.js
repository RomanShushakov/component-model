class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

export const divideJco = {
  divideJco(a, b) {
    if (b === 0.0) {
      throw new ComponentError("From divider-jco: Denominator is equal to zero!");
    }
    return a / b;
  }
};
