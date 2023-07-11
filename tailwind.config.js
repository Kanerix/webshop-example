/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.rs"],
	theme: {
		extend: {
			keyframes: {
				appear: {
					'0%': { transform: 'translateY(0.25rem);', opacity: '0;' },
					'100%': { transform: 'translateY(0px);', opacity: '1;' },
				},
				disappear: {
					'0%': { transform: 'translateY(0px);', opacity: '1;' },
					'100%': { transform: 'translateY(0.25rem);', opacity: '0;', visibility: 'hidden' },
				},
			},
			animation: {
				appear: 'appear 0.2s ease-in forwards',
				disappear: 'disappear 0.15s ease-out forwards',
			},
		},
	},
	plugins: [],
}
