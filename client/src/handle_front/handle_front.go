package main

import (
	sendmessage "bb-hackathon/tcp-chat.git/src/send_message"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

func CORSHandler(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization, Access-Control-Allow-Origin")
		if r.Method == "OPTIONS" {
			w.WriteHeader(http.StatusOK)
			return
		}
		next.ServeHTTP(w, r)
	})
}

type Message struct {
	Message string `json:"message"`
	UUID    string `json:"uuid"`
}

type UserCreds struct {
	Login    string `json:"login"`
	Password string `json:"password"`
}

type Usernames struct {
	Room      string   `json:"room"`
	Usernames []string `json:"usernames"`
}

type Room struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

func sendMessageHandler(w http.ResponseWriter, r *http.Request) {
	var text Message
	err := json.NewDecoder(r.Body).Decode(&text)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	sendmessage.SendMessage(text.Message, text.UUID)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}
func registerHandler(w http.ResponseWriter, r *http.Request) {
	var msg UserCreds
	err := json.NewDecoder(r.Body).Decode(&msg)
	username := msg.Login
	password := msg.Password
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fmt.Println(username, password)
	sendmessage.Register(username, password)
	sendmessage.Login(username, password)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

func loginHandler(w http.ResponseWriter, r *http.Request) {
	var msg UserCreds
	err := json.NewDecoder(r.Body).Decode(&msg)
	username := msg.Login
	password := msg.Password
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fmt.Println(username, password)
	sendmessage.Login(username, password)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

func createroomHandler(w http.ResponseWriter, r *http.Request) {
	var usernames Usernames
	err := json.NewDecoder(r.Body).Decode(&usernames)
	fmt.Println(usernames)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fmt.Println(usernames.Room)

	var usernames2 []string

	for _, value := range usernames.Usernames {
		usernames2 = append(usernames2, sendmessage.LookUpUser(value))
	}
	sendmessage.CreateRoom(usernames2, usernames.Room)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

func spitRooms(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	info := sendmessage.ListRooms()
	var rooms []Room
	for id, name := range info {
		rooms = append(rooms, Room{ID: id, Name: name})
	}
	json.NewEncoder(w).Encode(rooms)
}

func streamMessages(w http.ResponseWriter, r *http.Request) {
	roomID := r.URL.Query().Get("room_id")
	if roomID == "" {
		http.Error(w, "room_id параметр отсутствует", http.StatusBadRequest)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	for {
		var messages = sendmessage.ReceiveMessage(roomID)
		fmt.Println(messages)
		json.NewEncoder(w).Encode(messages)
	}
}

func handleGetMessages(w http.ResponseWriter, r *http.Request) {
	roomID := r.URL.Query().Get("room_id")
	if roomID == "" {
		http.Error(w, "room_id параметр отсутствует", http.StatusBadRequest)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	var messages = sendmessage.ListMessages(roomID)
	json.NewEncoder(w).Encode(messages)

}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/send", sendMessageHandler)
	mux.HandleFunc("/register", registerHandler)
	mux.HandleFunc("/login", loginHandler)
	mux.HandleFunc("/createroom", createroomHandler)
	mux.HandleFunc("/spitroom", spitRooms)
	mux.HandleFunc("/spitmessages", handleGetMessages)
	mux.HandleFunc("/streammessages", streamMessages)
	handler := CORSHandler(mux)

	fmt.Println("Server started at :8080")
	log.Fatal(http.ListenAndServe(":8080", handler))
	//sendmessage.Login("обезьяна", "обезьяна")
	// sendmessage.ListRooms()
	// sendmessage.SendMessage("Бобр1", "2e07b578-f550-426d-a204-9e7082665d2b")
	// sendmessage.SendMessage("Бобр2", "2e07b578-f550-426d-a204-9e7082665d2b")
}
