chrome.runtime.onStartup.addListener
(
    () =>
    {
        initializeExceptionLogic()

    },
)

chrome.tabs.onCreated.addListener
(
    (
        _tab,
    ) =>
    {
        initializeExceptionLogic()
    },
)

const initializeExceptionLogic = () =>
{
    chrome.declarativeNetRequest.updateSessionRules
    (
        {
            addRules: [
                dropXFrameAndCsp(),
            ],
        },
        () =>
        {
            if (chrome.runtime.lastError)
            {
                console.log(
                    chrome.runtime.lastError.message,
                )
            }
        },
    )
}

const dropXFrameAndCsp = () =>
{
    return {
        id: 1,
        priority: 1,
        action: {
            type: 'modifyHeaders',
            responseHeaders: [
                {
                    header: 'x-frame-options',
                    operation: 'remove',
                },
                {
                    header: 'content-security-policy',
                    operation: 'remove',
                },
            ],
        },
        condition: {
            urlFilter: `*${'https://news.ycombinator.com/*'}*`,
            resourceTypes: [
                'main_frame',
                'sub_frame',
            ],
        },
    }
}
