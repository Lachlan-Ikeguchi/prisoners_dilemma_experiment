-- name: BACKSTABBER
-- cooperates for 10 rounds then defects forever

function decide(round, history)
    if round <= 10 then
        return "cooperate"
    else
        return "defect"
    end
end
