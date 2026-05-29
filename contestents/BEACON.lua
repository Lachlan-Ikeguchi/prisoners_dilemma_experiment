-- name: BEACON
-- sends a signal by cooperating for 3, defecting for 1, repeating

function decide(round, history)
    local cycle = (round - 1) % 4
    if cycle < 3 then
        return "cooperate"
    else
        return "defect"
    end
end
