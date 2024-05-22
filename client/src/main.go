package main

import (
	sendmessage "bb-hackathon/tcp-chat.git/src/send_message"
)

func main() {
	sendmessage.Login("bobrik", "bobrik")
	//sendmessage.ListRooms()
	//sendmessage.SubscribeToUser()
	//sendmessage.ReceiveMessage("1e3f536e-9948-4980-b09a-7c54032ae6c2")
	//sendmessage.SendMessage("Дима ЛОХ!!!", "bcf4ca3f-54ee-4137-916b-4b973ad4f8c6")
	value := sendmessage.LookUpUser("klausr")
	sendmessage.CreateRoom([]string{value})
	//sendmessage.ListMessages("1e3f536e-9948-4980-b09a-7c54032ae6c2")
	//sendmessage.LookUpUser("klausr")
}
