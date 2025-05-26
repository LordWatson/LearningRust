document.getElementById('get-message').addEventListener('click', async () => {
    const messageElement = document.getElementById('message');
    messageElement.textContent = "Loading...";

    try {
        const response = await fetch('/api/hello');

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const contentType = response.headers.get('content-type');
        if (!contentType || !contentType.includes('application/json')) {
            throw new Error("Response is not JSON");
        }

        const data = await response.json();
        messageElement.textContent = data.text;
    } catch (error) {
        console.error('Fetch error:', error);
        messageElement.textContent = `Error: ${error.message}`;
    }
});