package api

import "net/http"

const FriendWeb = "http://localhost:3000"

func EachHandlerRoutine(w http.ResponseWriter) {
  w.Header().Add("Access-Control-Allow-Origin", FriendWeb)
  w.Header().Add("Access-Control-Allow-Credentials", "true")
}

func CreateResponseCROPS(
  w http.ResponseWriter, allowedMethods string,
) {
  w.Header().Add("Access-Control-Allow-Method", allowedMethods)
  w.Header().Add("Access-Control-Allow-Headers", "Content-type")
  w.WriteHeader(http.StatusOK)
}
