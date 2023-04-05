# Use an official Python runtime as a parent image
FROM python:3.11-slim

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY ./app /app
COPY requirements.txt /app

# Install the required packages from requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Make port 80 available to the world outside this container
EXPOSE 25566

# Run the command to start the app
CMD ["uvicorn", "app.controller:app", "--host", "127.0.0.1", "--port", "25566"]
