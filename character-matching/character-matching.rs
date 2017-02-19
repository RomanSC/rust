// character-matching.rs | Sun, Feb 12, 2017 | Roman S. Collins
//
// Not my code. Credit to Leo Tindall on YouTube, just following along:
// https://www.youtube.com/watch?v=7D9GE3-o54://www.youtube.com/watch?v=7D9GE3-o54o
//

enum MachineState {
    Normal,
    Comment,
    Upper,
    Lower,
}
