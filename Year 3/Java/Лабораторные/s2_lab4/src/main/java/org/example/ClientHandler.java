package org.example;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.Socket;

public class ClientHandler implements Runnable{
    private Socket socket;
    private String username;
    private PrintWriter out;
    private BufferedReader in;

    public ClientHandler(Socket socket) {
        this.socket = socket;
    }

    public String getUsername() {
        return username;
    }

    @Override
    public void run() {
        try {
            in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
            out = new PrintWriter(socket.getOutputStream(), true);

            out.println("Enter your username: ");
            username = in.readLine();
            out.flush();

            Server.registerClient(username, this);
            System.out.println(username + " joined.");
            Server.broadcast(username + " has joined the chat.", username);

            String message;
            while ((message = in.readLine()) != null) {
                if (message.equals("/clients")) {
                    Server.listClients(username);
                } else {
                System.out.println(username + ": " + message);
                Server.broadcast(username + ": " + message, username);

                }
            }

        } catch (IOException e) {
            System.out.println(username + " disconnected.");
        } finally {
            try {
                Server.removeClient(username);
                Server.broadcast(username + " has left the chat.", username);
                socket.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }

    public void sendMessage(String message) {
        out.println(message);
    }
}
