-- name: BETTER SAMARITAN
-- cooperates unless opponent defected in the last 3 rounds

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    -- Check last 3 moves
    local start = math.max(1, #history - 2)
    for i = start, #history do
        if history[i] == "defect" then
            return "defect"
        end
    end
    
    return "cooperate"
end
