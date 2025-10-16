package org.example;

import java.io.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.Scanner;

public class Client {
    public static final int PORT = 12345;

    BufferedReader in;
    PrintWriter out;
    BufferedReader console;

    public void connectToServer() throws IOException {
        makeConnections();
    }

    private void makeConnections() throws IOException {
        try (Socket socket = new Socket("localhost", PORT)) {
            System.out.println("Connected at " + socket);
            initializeIo(socket);

            new Thread(() -> {
                String serverMessage;
                try {
                    while ((serverMessage = in.readLine()) != null) {
                        System.out.println(serverMessage);
                    }
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }).start();

            String userInput;
            while ((userInput = console.readLine()) != null) {
                out.println(userInput);
                System.out.println("Server says: " + in.readLine());
            }
        }
    }

    public void initializeIo(Socket socket) throws IOException {
        in = createReader(socket);
        out = createWriter(socket);
        console = new BufferedReader(new InputStreamReader(System.in));
    }

    public static BufferedReader createReader(Socket socket) throws IOException {
        return new BufferedReader(new InputStreamReader(socket.getInputStream()));
    }

    private static PrintWriter createWriter(Socket socket) throws IOException {
        return new PrintWriter(socket.getOutputStream(), true);
    }
}
