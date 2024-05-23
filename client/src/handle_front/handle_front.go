package main

import (
	sendmessage "bb-hackathon/tcp-chat.git/src/send_message"
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
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
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fmt.Println(usernames.Usernames)

	var usernames2 []string

	for _, value := range usernames.Usernames {
		usernames2 = append(usernames2, sendmessage.LookUpUser(value))
	}
	fmt.Println(usernames2)
	sendmessage.CreateRoom(usernames2)

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

	jsonData, err := json.Marshal(rooms)
	if err != nil {
		fmt.Printf("Error occurred during marshaling. Err: %v\n", err)
		return
	}

	resp, err := http.Post("/spitroom", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Printf("Error occurred during sending request. Err: %v\n", err)
		return
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Printf("Error occurred during reading response. Err: %v\n", err)
		return
	}

	fmt.Printf("Response: %s\n", body)
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/send", sendMessageHandler)
	mux.HandleFunc("/register", registerHandler)
	mux.HandleFunc("/login", loginHandler)
	mux.HandleFunc("/createroom", createroomHandler)
	mux.HandleFunc("/spitroom", spitRooms)
	handler := CORSHandler(mux)

	fmt.Println("Server started at :8080")
	log.Fatal(http.ListenAndServe(":8080", handler))
	// sendmessage.Login("обезьяна", "обезьяна")
	// sendmessage.ListRooms()
}
