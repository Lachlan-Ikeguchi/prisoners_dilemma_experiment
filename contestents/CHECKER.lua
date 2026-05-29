-- name: CHECKER
-- cooperates on black squares, defects on white (chessboard pattern)

function decide(round, history)
    if (math.floor((round - 1) / 8) + (round - 1) % 8) % 2 == 0 then
        return "cooperate"
    else
        return "defect"
    end
end
