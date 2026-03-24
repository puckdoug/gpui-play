## TDD Workflow

This project follows strict test-driven development (red-green-refactor):

1. **Red phase**: Write failing tests first. Run them to confirm they fail. **STOP** and present the failures to the user. Do not proceed to implementation.
2. The user will review the test design, then commit the failing tests.
3. **Green phase**: Only after the user confirms, implement the minimum code to make the tests pass.
4. **Refactor phase**: Clean up if needed.

### Rules

- Never skip the pause between red and green phases.
- If tests pass immediately (not actually failing), flag this to the user — they are not valid red-phase tests.
- Write tests for all layers (pure state AND view/integration) before implementing each layer.
- The user commits the red phase to preserve test-first intent in git history.
