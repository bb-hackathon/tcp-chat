package form

import (
	pb "bb-hackathon/tcp-chat.git/proto"
	"context"
	"net/http"
	"os"
	"text/template"
)

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	fileContent, err := os.ReadFile("../form/form.html")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	tmpl, err := template.New("index").Parse(string(fileContent))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	err = tmpl.Execute(w, nil)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
}

func CreateHandler(client pb.RoomManagerClient) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Парсим данные из формы
		err := r.ParseForm()
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		// Получаем данные из формы
		roomID := r.FormValue("roomID")
		user1 := r.FormValue("user1")
		user2 := r.FormValue("user2")

		// Выполняем создание комнаты с использованием gRPC
		ctx := context.Background()
		room := &pb.Room{
			Uuid: &pb.UUID{Uuid: roomID},
			Users: []*pb.User{
				{
					Uuid:     &pb.UUID{Uuid: "user-uuid-1"},
					Nickname: user1,
				},
				{
					Uuid:     &pb.UUID{Uuid: "user-uuid-2"},
					Nickname: user2,
				},
			},
		}

		response, err := client.Create(ctx, room)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		// Отправляем ответ клиенту
		w.Write([]byte("Комната успешно создана! UUID комнаты: " + response.Uuid))
	}
}
