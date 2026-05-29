-- name: DOUBLE CROSS
-- cooperates twice, then defects once, repeating

function decide(round, history)
    local cycle = (round - 1) % 3
    if cycle < 2 then
        return "cooperate"
    else
        return "defect"
    end
end
