-- name: CALCULATOR
-- calculates optimal move based on remaining rounds

function decide(round, history)
    if round == 1 then return "cooperate" end
    
    local remaining = 100 - round
    if remaining < 5 then
        return "defect"
    end
    
    return history[#history]
end
