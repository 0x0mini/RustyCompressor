export $(grep -v '^#' .env | xargs)

echo "Building RustyCompressor..."
if cargo build --release; then
    echo "Build successful."
else
    echo "Build failed. Check for errors and try again."
    exit 1
fi

echo "Starting RustyCompressor..."
if cargo run --release; then
    echo "RustyCompressor is running on port $PORT"
else
    echo "Failed to start RustyCompressor."
    exit 1
fi
