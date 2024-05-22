package main

import (
	"bb-hackathon/tcp-chat.git/src/common"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

func CORSHandler(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS, PUT, DELETE")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		if r.Method == "OPTIONS" {
			return
		}

		next.ServeHTTP(w, r)
	})
}

type Message struct {
	Message string `json:"message"`
}

func sendMessageHandler(w http.ResponseWriter, r *http.Request) {
	var text Message
	err := json.NewDecoder(r.Body).Decode(&text)
	t := text.Message
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fmt.Println(t)
	common.SendMessage(t, "36537ef0-ee3e--aaa1-6547e466dd4a")

	// Ответ клиенту
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/send", sendMessageHandler)

	// Добавляем CORSHandler
	handler := CORSHandler(mux)

	fmt.Println("Server started at :8080")
	log.Fatal(http.ListenAndServe(":8080", handler))
}
