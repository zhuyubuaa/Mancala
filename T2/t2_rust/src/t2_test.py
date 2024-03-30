import numpy as np


def playermove():
    global player
    global choice
    global mod
    ban = 0
    if player == 2:
        ban = 6
    else:
        ban = 13
    rest = sit[choice]
    sit[choice] = 0
    while not rest == 0:
        choice = choice + 1
        if not choice % 14 == ban:
           sit[choice % 14] = sit[choice % 14] + 1
           rest = rest - 1
    return choice % 14


if __name__ == "__main__":

    # player1 score sit[6]
    # sit[0]-sit[5]
    # player2 score sit[13]
    # sit[7]-sit[12]

    # 12 11 10 9 8 7 player2
    # 0 1 2 3 4 5 player1

    # finish
    # middle
    # mix

    # extreme
    # min/max

    # error
    # 1:random,2:extreme,3:error
    mod = int(input("模式")) #mod1：1-3，mod2：4-6，mod3：7,8,9,10
    #7:该换没换
    #10:不该换但换了
    init = 0
    for j in range(1,100):
        player = np.random.randint(1, 3)
        init = player
        seq = []
        sit = np.array([4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0])
        index = -1
        half = False
        r = np.random.randint(0,2)
        errorPos = -1
        if mod == 2 or (mod == 3 and r == 1):
            half = True
        while (not np.sum(sit[0:6]) == 0) and (not np.sum(sit[7:13]) == 0):
            arr = []
            index = index + 1
            if half:
                if index == 5:
                    break
            if player == 1:
                for i in range(0, 6):
                    if not sit[i] == 0:
                        arr.append(i)
            else:
                for i in range(7,13):
                    if not sit[i] == 0:
                        arr.append(i)
            pos = 0
            choice = 0
            if mod == 4:
                ml = []
                max_val = 0
                for a in arr:
                    if sit[a] > max_val:
                        max_val = sit[a]
                for index, a in enumerate(arr):
                    if sit[a] == max_val:
                        ml.append(index)
                pos = np.random.randint(0, len(ml))
                choice = arr[ml[pos]]
            elif mod == 5:
                ml = []
                min_val = 48
                for a in arr:
                    if sit[a] < min_val:
                        min_val = sit[a]
                for index,a in enumerate(arr):
                    if sit[a] == min_val:
                        ml.append(index)
                pos = np.random.randint(0, len(ml))
                choice = arr[ml[pos]]
            elif mod == 9:
                if player == 1:
                    pos = np.random.randint(0,6)
                    if sit[pos] == 0:
                        if errorPos == -1:
                            errorPos = index
                else:
                    pos = np.random.randint(7,12)
                    if sit[pos] == 0:
                        if errorPos == -1:
                            errorPos = index
                choice = pos
            else:
                pos = np.random.randint(0, len(arr))
                choice = arr[pos]
            #print("player: ", player)
            if player == 1:
                #print("choice: ",choice + 1)
                seq.append(player * 10 + choice + 1)

            else:
                #print("choice: ", choice - 6)
                seq.append(player * 10 + choice - 6)
            last = playermove()
            if player == 1:
                if last == 6:
                    player = 1
                else:
                    if 0 <= last <= 5 and sit[last] == 1 and (not sit[12 - last] == 0):
                        sit[6] = sit[6] + sit[last] + sit[12-last]
                        sit[last] = 0
                        sit[12-last] = 0
                    player = 2
            else:
                if last == 13:
                    player = 2
                else:
                    if 7 <= last <= 12 and sit[last] == 1 and (not sit[12 - last] == 0):
                        sit[13] = sit[13] + sit[last] + sit[12 - last]
                        sit[last] = 0
                        sit[12 - last] = 0
                    player = 1
            #print("player2: ", sit[12:6:-1])
            #print("player1: ", sit[:6])
            #print("score2: ", sit[13])
            #print("score1: ", sit[6])
        if not half:
            sit[13] = sit[13] + np.sum(sit[7:13])
            sit[6] = sit[6] + np.sum(sit[0:6])
        if mod == 8:
            seq.append(14)
            errorPos = len(seq) - 1

        illegal = False
        if mod == 7: #ABA
            cnt = 0
            while True:
                seed = np.random.randint(0,len(seq)-3)
                if seq[seed] // 10 == seq[seed + 2] // 10 and seq[seed] // 10 != seq[seed + 1] // 10:
                    temp = seq[seed + 2]
                    seq[seed + 2] = seq[seed] + 1
                    seq[seed + 1] = temp
                    if errorPos == -1:
                        errorPos = seed + 1
                    break
                cnt = cnt + 1
                if cnt >= 100:
                    illegal = True
                    break
        if mod == 10: #AAB
            cnt = 0
            while True:
                seed = np.random.randint(0,len(seq)-3)
                if seq[seed] // 10 == seq[seed + 1] // 10 and seq[seed + 2] // 10 != seq[seed + 1] // 10:
                    temp = seq[seed + 2]
                    seq[seed + 2] = seq[seed] + 1
                    seq[seed + 1] = temp
                    if errorPos == -1:
                        errorPos = seed + 1
                    break
                cnt = cnt + 1
                if cnt >= 100:
                    illegal = True
                    break
        if errorPos == -1:
            #print("final-score2: ", sit[13])
            #print("final-score1: ", sit[6])
            print("  #[test]");
            print("  fn test" + str(j) + "()" + " {");
            if not half:
                print(f'      assert_eq!(mancala_result({init}, &{seq},{len(seq)}), {15000 + (sit[6] - sit[13] if init == 1 else sit[13] - sit[6])});')
            else:
                print(
                    f'      assert_eq!(mancala_result({init}, &{seq},{len(seq)}), {20000 + (sit[6] if init == 1 else sit[13] )});')
            print("  }")
        else:
            print("  #[test]");
            print("  fn test" + str(j) + "()" + " {");
            print(
                    f'      assert_eq!(mancala_result({init}, &{seq},{len(seq)}), {30000 + errorPos});')
            print("  }")
