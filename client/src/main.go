package main

import (
	sendmessage "bb-hackathon/tcp-chat.git/src/send_message"
)

func main() {
	sendmessage.Login("klausr", "klausr")
	sendmessage.ListRooms()
	sendmessage.SubscribeToUser()
}
