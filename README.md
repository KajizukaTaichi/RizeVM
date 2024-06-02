# Stem
The simple emulator of turing machine.

The name is recursive acronym of "Stem: Turing Educational Machine".
It's designed use for STEM education.

```bash
$ stem ./transitions.json
Stem: Turing Educational Machine
-----------------------------------
Current State: q0, Read: 0
[0] 0 0 0 0 0 0 0 0 0
Write: 0, Move: "Right", Next State: q1
 0[0] 0 0 0 0 0 0 0 0

Current State: q1, Read: 0
 0[0] 0 0 0 0 0 0 0 0
Write: 1, Move: "Right", Next State: q0
 0 1[0] 0 0 0 0 0 0 0

Current State: q0, Read: 0
 0 1[0] 0 0 0 0 0 0 0
Write: 0, Move: "Right", Next State: q1
 0 1 0[0] 0 0 0 0 0 0

Current State: q1, Read: 0
 0 1 0[0] 0 0 0 0 0 0
Write: 1, Move: "Right", Next State: q0
 0 1 0 1[0] 0 0 0 0 0

Current State: q0, Read: 0
 0 1 0 1[0] 0 0 0 0 0
Write: 0, Move: "Right", Next State: q1
 0 1 0 1 0[0] 0 0 0 0

Current State: q1, Read: 0
 0 1 0 1 0[0] 0 0 0 0
Write: 1, Move: "Right", Next State: q0
 0 1 0 1 0 1[0] 0 0 0

Current State: q0, Read: 0
 0 1 0 1 0 1[0] 0 0 0
Write: 0, Move: "Right", Next State: q1
 0 1 0 1 0 1 0[0] 0 0

Current State: q1, Read: 0
 0 1 0 1 0 1 0[0] 0 0
Write: 1, Move: "Right", Next State: q0
 0 1 0 1 0 1 0 1[0] 0

Current State: q0, Read: 0
 0 1 0 1 0 1 0 1[0] 0
Write: 0, Move: "Right", Next State: q1
 0 1 0 1 0 1 0 1 0[0]

Current State: q1, Read: 0
 0 1 0 1 0 1 0 1 0[0]
Write: 1, Move: "Right", Next State: q0
 0 1 0 1 0 1 0 1 0 1[_]
```

## transitions.json
```json
{
    "tape": "0000000000",
    "start_state": "q0",
    "transitions": {
        "q0_0": { "write": "0", "direction": "Right", "next_state": "q1" },
        "q1_0": { "write": "1", "direction": "Right", "next_state": "q0" }
    }
}
```