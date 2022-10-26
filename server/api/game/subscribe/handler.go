package subscribe

import (
	"net/http"

	"checkers/server/api"
	"checkers/server/internal/data"
	"checkers/server/internal/errorsStrings"
)

func Handler(w http.ResponseWriter, r *http.Request) {
	api.EachHandlerRoutine(w)
	if r.Method == http.MethodOptions {
		api.CreateResponseCROPS(w, "GET, POST, PUT, DELETE")
		return
	}
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}

	var token string
	cookies := r.Cookies()
	for _, c := range cookies {
		if c.Name == "token" {
			token = c.Value
		}
	}

	gameName := r.URL.Query().Get("gamename")

	storage := data.GetGlobalStorage()
	callback := func(data []byte) {
		if data == nil {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		w.Write(data)
		w.WriteHeader(http.StatusOK)
	}
	err := storage.OnChangeGame(token, gameName, callback)
	if err == nil {
		return
	}
	switch err.Error() { //refactor extract method
	case errorsStrings.NotAuthorized:
		w.WriteHeader(http.StatusUnauthorized)
	case errorsStrings.NotFound:
		w.WriteHeader(http.StatusNotFound)
	case errorsStrings.PermissionDenied:
		w.WriteHeader(http.StatusForbidden)
	default:
		w.WriteHeader(http.StatusInternalServerError)
	}
}