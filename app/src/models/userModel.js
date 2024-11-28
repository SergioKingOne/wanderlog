const { Sequelize, DataTypes } = require("sequelize");
const { sequelize } = require("../utils/db");

const User = sequelize.define(
  "User",
  {
    // Model attributes
    id: {
      type: DataTypes.INTEGER,
      primaryKey: true,
      autoIncrement: true,
    },
    name: {
      type: DataTypes.STRING,
      allowNull: false,
    },
    email: {
      type: DataTypes.STRING,
      allowNull: false,
      unique: true,
    },
  },
  {
    // Other model options
    tableName: "users",
    timestamps: true,
  }
);

module.exports = { User };
