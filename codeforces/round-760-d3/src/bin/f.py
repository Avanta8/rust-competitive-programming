def main():
    a, b = map(int, input().split())

    ba = bin(a)[2:]
    bb = bin(b)[2:]

    if ba == bb:
        return True

    if bb[-1] == "0":
        return False

    rba = ba[::-1]

    for ta in (ba, rba, "1" + rba):
        ta = ta.lstrip("0")
        if ta not in bb:
            continue
        start = bb.index(ta)
        if (not bb[:start] or set(bb[:start]) == {"1"}) and (
            not bb[start + len(ta) :] or set(bb[start + len(ta) :]) == {"1"}
        ):
            return True

    return False


print("YES" if main() else "NO")
