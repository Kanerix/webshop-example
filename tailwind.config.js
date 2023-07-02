/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./src/**/*.rs"],
	theme: {
		extend: {
			keyframes: {
				slideIn: {
					'0%': { transform: 'translateY(-100%)' },
					'100%': { transform: 'translateY(0)' },
				},
			},
			animation: {
				slideIn: 'slideIn 0.3s ease-in-out',
			},
		},
	},
	plugins: [],
}
