package org.example;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.concurrent.ConcurrentHashMap;

public class Server {
    private static final int PORT = 12345;
    private static final Map<String, ClientHandler> clients = new ConcurrentHashMap<>();

    public static void start() {
        try (ServerSocket serverSocket = new ServerSocket(PORT)) {
            System.out.println("Server started on port " + PORT);

            while (true) {
                Socket socket = serverSocket.accept();
                System.out.println("New client connected: " + socket.getInetAddress());

                // Handle client in a separate thread
                new Thread(new ClientHandler(socket)).start();
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    // Broadcast a message to all connected clients
    public static void broadcast(String message, String senderUsername) {
        for (ClientHandler client : clients.values()) {
            if (!Objects.equals(client.getUsername(), senderUsername)) {
                client.sendMessage(message);
            }
        }
    }

    public static void listClients(String sender) {
        ClientHandler client = clients.get(sender);

        StringBuilder builder = new StringBuilder("Clients: ");
        for (ClientHandler clientInList : clients.values()) {
            builder.append(clientInList.getUsername()).append(";");
        }
        client.sendMessage(builder.toString());
    }

    // Add client to map
    public static void registerClient(String username, ClientHandler handler) {
        clients.put(username, handler);
    }

    // Remove client from map
    public static void removeClient(String username) {
        clients.remove(username);
    }
}
