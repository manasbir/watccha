## wallet monitor

ideally we want some sort of plug and play thing
where everything is modular and users can just plug in whatever they want

will prob add a settings.txt and there would be a setup command where users can choose commands from a menu (will be fun)

some ideas for tracking
 - erc20s
 - uniswap trades
 - LP positions
 - liquidations
 - general DeFi (gmx, aave, uniswap), forks should work automatically
 - calldata decoder with gpt, sig.samczsun, and try etherscan (also will be fun)

 roadmap:
  - add discord & tg bot
  - add more event listeners
  - make better tui
  - add more commands
  - block list (dont decode spam tx things)