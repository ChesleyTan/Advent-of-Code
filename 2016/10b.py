import sys

class Bot(object):
    def __init__(self, _id):
        self.id = _id
        self.lower = None
        self.higher = None
        self.chips = []

bots = {}
outputs = {}
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
                if bot.lower[0] == "output":
                    output_id = int(bot.lower[1])
                    if output_id not in outputs:
                        outputs[output_id] = [lower]
                    else:
                        outputs[output_id].append(lower)
                else:
                    bots[int(bot.lower[1])].chips.append(lower)
                if bot.higher[0] == "output":
                    output_id = int(bot.higher[1])
                    if output_id not in outputs:
                        outputs[output_id] = [higher]
                    else:
                        outputs[output_id].append(higher)
                else:
                    bots[int(bot.higher[1])].chips.append(higher)
                bot.chips = []
        if not progress:
            break
    print outputs
    print outputs[0][0] * outputs[1][0] * outputs[2][0]
