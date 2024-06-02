# RizeVM
Turing model virtual machine for education

## Define transitions
```json
{
  "start_state": "q0",
  "transitions": {
    "q0_0": { "write": "1", "direction": "Right", "next_state": "q1" },
    "q1_0": { "write": "0", "direction": "Left",  "next_state": "q2" },
    "q2_1": { "write": "1", "direction": "Right", "next_state": "q0" },
    "q3_1": { "write": "0", "direction": "Left",  "next_state": "q0" }
  }
}
```
## Output

```
RizeVM
Turing model virtual machine for education


Current State: q0, Read: 0
[0] 0 0
Write: 1, Move: "Right", Next State: q1
 1[0] 0

Current State: q1, Read: 0
 1[0] 0
Write: 0, Move: "Left", Next State: q2
[1] 0 0

Current State: q2, Read: 1
[1] 0 0
Write: 1, Move: "Right", Next State: q0
 1[0] 0

Current State: q0, Read: 0
 1[0] 0
Write: 1, Move: "Right", Next State: q1
 1 1[0]

Current State: q1, Read: 0
 1 1[0]
Write: 0, Move: "Left", Next State: q2
 1[1] 0

Current State: q2, Read: 1
 1[1] 0
Write: 1, Move: "Right", Next State: q0
 1 1[0]

Current State: q0, Read: 0
 1 1[0]
Write: 1, Move: "Right", Next State: q1
 1 1 1[_]
```

