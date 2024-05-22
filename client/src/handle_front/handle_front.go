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
}
type UserCreds struct {
	Login    string `json:"login"`
	Password string `json:"password"`
}

func sendMessageHandler(w http.ResponseWriter, r *http.Request) {
	var text Message
	err := json.NewDecoder(r.Body).Decode(&text)
	t := text.Message
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	sendmessage.SendMessage(t, "8299ace8-e565-497a-868a-e48fde731fef")

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

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/send", sendMessageHandler)
	mux.HandleFunc("/register", registerHandler)
	mux.HandleFunc("/login", loginHandler)
	// Добавляем CORSHandler
	handler := CORSHandler(mux)

	fmt.Println("Server started at :8080")
	log.Fatal(http.ListenAndServe(":8080", handler))
}
