struct QuitMessage;
struct MoveMessage {
  x: i32,
  y: i32,
}

struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct