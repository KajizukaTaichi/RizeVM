# Stem
The simple emulator of turing machine.

The name is recursive acronym of "Stem: Turing Educational Machine".
It's designed use for STEM education.

## Specification
- "tape": include UTF-8 character as symbol
- "start_state": initial state when start run
- "transitions": define rules of transitions

### Key format of transitions
Write split "_" underscore.
The left designate state, the right designate symbol will read

## Example

This is to reverse binary value.
```
{
    "tape": "11010010",
    "start_state": "q0",
    "transitions": {
        "q0_0": { "write": "1", "direction": "Right", "next_state": "q0" },
        "q0_1": { "write": "0", "direction": "Right", "next_state": "q0" }
    }
}
```

```bash
Stem: Turing Educational Machine
-----------------------------------
Current State: q0, Read: 1
[1] 1 0 1 0 0 1 0
Write: 0, Move: Right, Next State: q0
 0[1] 0 1 0 0 1 0

Current State: q0, Read: 1
 0[1] 0 1 0 0 1 0
Write: 0, Move: Right, Next State: q0
 0 0[0] 1 0 0 1 0

Current State: q0, Read: 0
 0 0[0] 1 0 0 1 0
Write: 1, Move: Right, Next State: q0
 0 0 1[1] 0 0 1 0

Current State: q0, Read: 1
 0 0 1[1] 0 0 1 0
Write: 0, Move: Right, Next State: q0
 0 0 1 0[0] 0 1 0

Current State: q0, Read: 0
 0 0 1 0[0] 0 1 0
Write: 1, Move: Right, Next State: q0
 0 0 1 0 1[0] 1 0

Current State: q0, Read: 0
 0 0 1 0 1[0] 1 0
Write: 1, Move: Right, Next State: q0
 0 0 1 0 1 1[1] 0

Current State: q0, Read: 1
 0 0 1 0 1 1[1] 0
Write: 0, Move: Right, Next State: q0
 0 0 1 0 1 1 0[0]

Current State: q0, Read: 0
 0 0 1 0 1 1 0[0]
Write: 1, Move: Right, Next State: q0
 0 0 1 0 1 1 0 1[_]
```