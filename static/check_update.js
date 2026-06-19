async function checkInstanceUpdateStatus() {
    const statusElement = document.getElementById('update-status');
    const commitElement = document.getElementById('git_commit');
    if (!statusElement || !commitElement) return;

    try {
        const response = await fetch('/commits.atom');
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        const text = await response.text();
        const parser = new DOMParser();
        const xmlDoc = parser.parseFromString(text, "application/xml");
        const entries = xmlDoc.getElementsByTagName('entry');
        const localCommit = commitElement.dataset.value.trim();

        let statusMessage = '';

        if (localCommit === '' || localCommit === 'dev') {
            statusMessage = '⚠️ Unable to determine this instance commit.';
        } else if (entries.length > 0) {
            const commitHashes = Array.from(entries).map(entry => {
                const id = entry.getElementsByTagName('id')[0].textContent;
                return id.split('/').pop().trim();
            });

            const commitIndex = commitHashes.indexOf(localCommit);

            if (commitIndex === 0) {
                statusMessage = '✅ Instance is up to date.';
            } else if (commitIndex > 0) {
                statusMessage = `⚠️ This instance is not up to date and is ${commitIndex} commits old. Test and confirm on an up-to-date instance before reporting.`;
                const error446 = document.getElementById('error-446');
                if (error446) error446.remove();
            } else {
                statusMessage = '⚠️ Unable to confirm this instance commit against this fork\'s latest commit feed.';
            }
        } else {
            statusMessage = '⚠️ Unable to fetch commit information.';
        }

        statusElement.innerText = statusMessage;
    } catch (error) {
        console.error('Error fetching commits:', error);
        statusElement.innerText = '⚠️ Error checking update status: ' + error;
    }
}

async function checkOtherInstances() {
    const randomInstanceElement = document.getElementById('random-instance');
    if (!randomInstanceElement) return;

    try {
        const response = await fetch('/instances.json');
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        const data = await response.json();
        const instances = window.location.host.endsWith('.onion') ? data.instances.filter(i => i.onion) : data.instances.filter(i => i.url);
        if (instances.length == 0) return;
        const randomInstance = instances[Math.floor(Math.random() * instances.length)];
        const instanceUrl = randomInstance.url ?? randomInstance.onion;
        // Set the href of the <a> tag to the instance URL with path included
        randomInstanceElement.href = instanceUrl + window.location.pathname;
        randomInstanceElement.innerText = "Visit Random Instance";
    } catch (error) {
        console.error('Error fetching instances:', error);
    }
}

// Set the target URL when the page loads
window.addEventListener('load', checkOtherInstances);

checkInstanceUpdateStatus();
