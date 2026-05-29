-- name: CAMELEON
-- changes to match opponent's recent behavior pattern

function decide(round, history)
    if round == 1 then return "cooperate" end
    if #history < 3 then return history[#history] end
    
    -- Check if opponent has a repeating pattern
    if history[#history] == history[#history-1] and history[#history-1] == history[#history-2] then
        return history[#history]
    end
    
    return "cooperate"
end
