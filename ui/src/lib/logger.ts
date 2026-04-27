import pino from 'pino';

export const logger = pino({
    level: process.env.LOG_LEVEL || 'info',
    // Samakan format dengan tracing-subscriber Rust
    timestamp: () => `,"timestamp":"${new Date().toISOString()}"`,
    formatters: {
        level: (label) => {
            return { level: label.toUpperCase() };
        },
    },
    messageKey: 'message',
    base: { target: 'narthex-ui' }
});
