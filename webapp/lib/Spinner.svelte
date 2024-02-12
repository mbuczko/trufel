<div class="lds-facebook">
	<div />
	<div />
	<div />
</div>

<style>
.lds-facebook {
	display: inline-block;
	position: relative;
	width: 50px;
	height: 24px;
}
.lds-facebook div {
	display: inline-block;
	position: absolute;
	left: 8px;
	width: 8px;
	background: #888;
	animation: lds-facebook 1.2s cubic-bezier(0, 0.5, 0.5, 1) infinite;
}
.lds-facebook div:nth-child(1) {
	left: 8px;
	animation-delay: -0.24s;
}
.lds-facebook div:nth-child(2) {
	left: 20px;
	animation-delay: -0.12s;
}
.lds-facebook div:nth-child(3) {
	left: 32px;
	animation-delay: 0;
}
@keyframes lds-facebook {
	0% {
		top: 2px;
		height: 24px;
	}
	50%,
	100% {
		top: 10px;
		height: 12px;
	}
}
</style>
