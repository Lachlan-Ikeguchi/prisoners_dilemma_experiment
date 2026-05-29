-- name: BALANCED
-- cooperates if opponent's coop rate > 55%

function decide(round, history)
    if round == 1 then return "cooperate" end
    local coop = 0
    for i = 1, #history do
        if history[i] == "cooperate" then coop = coop + 1 end
    end
    return coop / #history > 0.55 and "cooperate" or "defect"
end
