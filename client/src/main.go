package main

import (
	sendmessage "bb-hackathon/tcp-chat.git/src/send_message"
)

func main() {
	sendmessage.Login("user1234", "1234")
	sendmessage.SendMessage("message", "8299ace8-e565-497a-868a-e48fde731fef")
}
