import sys

class Bot(object):
    def __init__(self, _id):
        self.id = _id
        self.lower = None
        self.higher = None
        self.chips = []

bots = {}
if __name__ == "__main__":
    with open("10a.input", 'r') as f:
        lines = f.read().strip().split('\n')
        for line in lines:
            tokens = line.split()
            if tokens[0] == "value":
                bot_id = int(tokens[5])
                chip = int(tokens[1])
                if bot_id not in bots:
                    bots[bot_id] = Bot(bot_id)
                bots[bot_id].chips.append(chip)
            elif tokens[0] == "bot":
                bot_id = int(tokens[1])
                if bot_id not in bots:
                    bots[bot_id] = Bot(bot_id)
                bots[bot_id].lower = (tokens[5], tokens[6])
                bots[bot_id].higher = (tokens[10], tokens[11])
    while True:
        progress = False
        for bot in bots.values():
            if len(bot.chips) == 2:
                progress = True
                lower = min(bot.chips)
                higher = max(bot.chips)
                if lower == 17 and higher == 61:
                    print bot.id
                    sys.exit(0)
                if bot.lower[0] == "output":
                    pass
                else:
                    bots[int(bot.lower[1])].chips.append(lower)
                if bot.higher[0] == "output":
                    pass
                else:
                    bots[int(bot.higher[1])].chips.append(higher)
                bot.chips = []
        if not progress:
            break
