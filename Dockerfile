# Use an official Python runtime as a parent image
FROM python:3.11-slim

# Copy the current directory contents into the container at /app
COPY ./ /
COPY requirements.txt /app

# Install the required packages from requirements.txt
RUN pip install --no-cache-dir -r app/requirements.txt

# Make port 25566 available to the world outside this container
EXPOSE 25566

# Run the command to start the app
CMD ["gunicorn", "app.controller:app", "--config", "app/gunicorn.conf.py"]
