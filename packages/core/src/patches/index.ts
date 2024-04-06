export const patches = {
  binaryName: () => import('./binary-name').then((r) => r.default || r),
}
