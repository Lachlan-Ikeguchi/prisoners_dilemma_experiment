-- name: FRENEMY
-- cooperates for 5 rounds, then defects for 3, repeating

function decide(round, history)
    local cycle = (round - 1) % 8
    if cycle < 5 then
        return "cooperate"
    else
        return "defect"
    end
end
