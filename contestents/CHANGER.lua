-- name: CHANGER
-- changes strategy mid-game based on score

function decide(round, history)
    if round <= 50 then
        return "cooperate"
    else
        return "defect"
    end
end
