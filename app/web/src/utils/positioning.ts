export function getNextPosition(position: string) {
  const lastChar = position.slice(-1);
  const nextChar = String.fromCharCode(lastChar.charCodeAt(0) + 1);
  return position.slice(0, -1) + nextChar;
}
