import { Server } from "socket.io";

const io = new Server({ /* options */ });

io.on("connection", (socket) => {
  socket.on("event", (data) => {
    console.log(data);
  })
  socket.on("event_with_ack", (data, callback) => {
    console.log(data);
    callback(data);
  })
});

io.listen(3000);
