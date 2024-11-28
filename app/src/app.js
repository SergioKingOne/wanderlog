require("dotenv").config();
const express = require("express");
const app = express();
const userRoutes = require("./routes/userRoutes");
const { sequelize, syncDatabase } = require("./utils/db");

// Middleware
app.use(express.json());

// Routes
app.use("/api/users", userRoutes);

// Health Check Route
app.get("/health", (req, res) => {
  res.status(200).json({ status: "OK" });
});

// Database Connection
sequelize
  .authenticate()
  .then(async () => {
    console.log("Database connected...");

    // Sync database
    await syncDatabase();

    // Start Server
    const PORT = process.env.PORT || 3000;
    app.listen(PORT, () => {
      console.log(`Server running on port ${PORT}`);
    });
  })
  .catch((err) => {
    console.error("Unable to connect to the database:", err);
  });

module.exports = app;
