# Kivo Music Quality Gates

Before delivery, every ticket must answer:

- Did any file become too large or multi-responsibility?
- Did any page absorb feature logic?
- Are mock data, copy, types, state, and styles separated?
- Does the change preserve feature boundaries?
- Does the change preserve accessibility basics?
- Does the change avoid fake audio-quality claims?
- Does the change avoid unrelated files?

Required local checks:

```bash
npm run typecheck
npm run build
```

Merging code that only works visually but breaks architecture is not acceptable.
