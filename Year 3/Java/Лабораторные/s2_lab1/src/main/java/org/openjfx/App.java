package org.openjfx;

import javafx.application.Application;
import javafx.scene.Scene;
import javafx.scene.canvas.Canvas;
import javafx.scene.canvas.GraphicsContext;
import javafx.scene.layout.StackPane;
import javafx.scene.paint.Paint;
import javafx.stage.Stage;
import javafx.scene.paint.Color;

import java.io.Console;
import java.util.Objects;


/**
 * JavaFX App
 */

public class App extends Application {

    private static boolean isMultiWindow = false;

    @Override
    public void start(Stage stage) {
        if (isMultiWindow) {
            drawMultiWindow();
        }
        else {
            drawSingleWindow(stage);
        }
    }

    public void drawMultiWindow() {
        createWindow("Red", Color.RED, Math.PI / 3, 100, 100);
        createWindow("Green", Color.GREEN, 2 * Math.PI / 3, 200, 200);
        createWindow("Blue", Color.BLUE, 0, 300, 300);
    }

    private void createWindow(String title, Paint color, double offset, int x, int y) {
        Stage newStage = new Stage();
        Canvas canvas = new Canvas(600, 600);
        GraphicsContext gc = canvas.getGraphicsContext2D();

        gc.setFill(Color.WHITE);
        gc.fillRect(0, 0, canvas.getWidth(), canvas.getHeight());

        Picture picture = new Picture(gc, color, offset, x, y);
        picture.start();

        StackPane root = new StackPane(canvas);
        Scene scene = new Scene(root, 600, 600);

        newStage.setTitle(title);
        newStage.setScene(scene);
        newStage.setResizable(false);
        newStage.setX(x);
        newStage.setY(y);
        newStage.show();
    }

    public void drawSingleWindow(Stage stage) {

        Canvas canvas = new Canvas(600, 600);
        GraphicsContext gc = canvas.getGraphicsContext2D();

        gc.setFill(Color.WHITE);
        gc.fillRect(0, 0, canvas.getWidth(), canvas.getHeight());

        Picture red = new Picture(gc, Color.RED, Math.PI / 3, 100, 100);
        Picture green = new Picture(gc, Color.GREEN, 2 * Math.PI / 3, 200, 200);
        Picture blue = new Picture(gc, Color.BLUE, 0, 300, 300);

        red.start();
        green.start();
        blue.start();

        StackPane root = new StackPane(canvas);
        Scene scene = new Scene(root, 600, 600);

        stage.setTitle("Test");
        stage.setScene(scene);
        stage.setResizable(false);
        stage.show();
    }

    public static void main(String[] args) {
        for (String arg : args) {
           System.out.println(arg);
        }
        if (args.length == 1 && Objects.equals(args[0], "--separate-windows")) {
            App.isMultiWindow = true;
        }
        launch();
    }
}

class Picture implements Runnable {
    private final GraphicsContext gc;
    private Paint color;
    private double rotationOffset;
    private int centerX;
    private int centerY;

    public Picture(GraphicsContext gc, Paint color, double rotationOffset, int x, int y) {
        this.gc = gc;
        this.color = color;
        this.rotationOffset = rotationOffset;
        this.centerX = x;
        this.centerY = y;
    }

    public void start() {
        new Thread(this).start();
    }

    @Override
    public void run() {
        drawSpiral();
    }

    public void drawSpiral() {
        double a = 100;

        synchronized (gc) {
            gc.setStroke(color);
            gc.setLineWidth(1.5);
            gc.beginPath();

            double startAngle = 0.1;
            double endAngle = 10 * Math.PI;
            double step = 0.01;

            boolean firstPoint = true;
            for (double phi = startAngle; phi <= endAngle; phi += step) {
                double r = a / phi;
                double x = centerX + r * Math.cos(phi + rotationOffset);
                double y = centerY + r * Math.sin(phi + rotationOffset);

                if (firstPoint) {
                    gc.moveTo(x, y);
                    firstPoint = false;
                } else {
                    gc.lineTo(x, y);
                }
            }

            gc.stroke();

        }
    }
}
