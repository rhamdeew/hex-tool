/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        // Light theme
        background: '#FFFFFF',
        surface: '#F7F7F7',
        border: '#E5E5E5',
        'text-primary': '#1A1A1A',
        'text-secondary': '#666666',
        accent: '#3B82F6',
        success: '#10B981',
        warning: '#F59E0B',
        error: '#EF4444',
        // Dark theme
        'dark-background': '#1A1A1A',
        'dark-surface': '#2D2D2D',
        'dark-border': '#404040',
        'dark-text-primary': '#F5F5F5',
        'dark-text-secondary': '#A3A3A3',
        'dark-accent': '#60A5FA',
        'dark-success': '#34D399',
        'dark-warning': '#FBBF24',
        'dark-error': '#F87171',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'SF Mono', 'monospace'],
      },
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
      },
      borderRadius: {
        DEFAULT: '8px',
      },
    },
  },
  plugins: [],
  darkMode: 'class',
}
